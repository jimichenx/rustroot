/**
 * 动图交互的共享基础设施。
 *
 * 只负责三件通用的事，具体编排留给各图自己：
 * 1. 进入视口播一次（`onFirstView`）；
 * 2. 减动效判定（`reducedMotion`）；
 * 3. 分步状态机（`createStepEngine`）——按 `data-step` 分组切 `is-on`。
 *
 * 约定：
 * - 图根元素带 `data-stepper`（进度条）或 `data-play`（播放按钮）；
 * - 每个步骤的元素带 `data-step="n"`（n 从 1 开始）；
 * - 只有脚本跑起来才给图加 `is-armed`，关掉 JS 时全部直接可见。
 *
 * 进度条 / 播放按钮的 DOM 由 `FigureControls.astro` 渲染，样式见 diagrams.css。
 */

/** 系统是否要求减弱动态效果。 */
export function reducedMotion() {
	return window.matchMedia('(prefers-reduced-motion: reduce)').matches;
}

/** 元素第一次进入视口时回调一次；浏览器没有 IntersectionObserver 时立即回调。 */
export function onFirstView(el, callback, threshold = 0.35) {
	if (!('IntersectionObserver' in window)) {
		callback();
		return;
	}
	const observer = new IntersectionObserver(
		(entries) => {
			if (!entries.some((entry) => entry.isIntersecting)) return;
			observer.disconnect();
			callback();
		},
		{ threshold }
	);
	observer.observe(el);
}

/**
 * 分步状态机：读一段 `[data-step]`，管理当前进度，切换 `is-on`。
 * 只切类、不管控件；控件在 `onRender` 里更新。
 */
function createStepEngine(figure, onRender) {
	const elements = [...figure.querySelectorAll('[data-step]')];
	const count = elements.reduce((max, el) => Math.max(max, Number(el.dataset.step) || 0), 0);
	let current = 0;
	let timer = null;

	const render = () => {
		elements.forEach((el) => el.classList.toggle('is-on', Number(el.dataset.step) <= current));
		// 图根带上当前步，供需要按步定位的元素用（如位置链的光标）。
		figure.dataset.step = String(current);
		if (onRender) onRender();
	};
	const stop = () => {
		if (timer) {
			clearTimeout(timer);
			timer = null;
		}
	};
	const set = (n) => {
		stop();
		current = Math.max(0, Math.min(count, n));
		render();
	};
	// 瞬间回到起点再从头一步步播，避免看到倒放。
	const play = (interval = 800, onDone) => {
		stop();
		figure.classList.add('is-resetting');
		current = 0;
		render();
		void figure.offsetWidth;
		figure.classList.remove('is-resetting');
		const tick = () => {
			current += 1;
			render();
			if (current < count) timer = setTimeout(tick, interval);
			else if (onDone) onDone();
		};
		timer = setTimeout(tick, 150);
	};

	return {
		count,
		get current() {
			return current;
		},
		set,
		play,
		stop,
		render,
	};
}

/** 进度条：节点跳步、上一步/下一步、自动播放，键盘 ←/→。 */
function bindStepper(figure) {
	const dots = [...figure.querySelectorAll('[data-step-target]')];
	const segs = [...figure.querySelectorAll('[data-seg]')];
	const prev = figure.querySelector('[data-step-prev]');
	const next = figure.querySelector('[data-step-next]');
	const auto = figure.querySelector('[data-step-auto]');
	const engine = createStepEngine(figure, () => {
		dots.forEach((dot) => {
			const n = Number(dot.dataset.stepTarget);
			dot.classList.toggle('is-on', n <= engine.current);
			dot.setAttribute('aria-current', n === engine.current ? 'step' : 'false');
		});
		segs.forEach((seg) => seg.classList.toggle('is-on', Number(seg.dataset.seg) < engine.current));
		if (prev) prev.disabled = engine.current <= 0;
		if (next) next.disabled = engine.current >= engine.count;
	});
	if (!engine.count || !dots.length) return;

	figure.classList.add('is-armed');
	engine.render();

	let playing = false;
	const setAuto = () => {
		if (!auto) return;
		const lbl = playing ? auto.dataset.labelPause : auto.dataset.labelAuto;
		auto.setAttribute('aria-pressed', String(playing));
		auto.setAttribute('aria-label', lbl);
		auto.setAttribute('title', lbl);
	};
	setAuto();

	// 手动操作先停掉自动播放，再落步。
	const setStep = (n) => {
		playing = false;
		setAuto();
		engine.set(n);
	};
	const playAuto = () => {
		playing = true;
		setAuto();
		engine.play(900, () => {
			playing = false;
			setAuto();
		});
	};

	dots.forEach((dot) => {
		dot.addEventListener('click', () => {
			const target = Number(dot.dataset.stepTarget);
			// 点到当前节点就回退一步，否则跳到该节点。
			setStep(target === engine.current ? target - 1 : target);
		});
	});
	segs.forEach((seg) => seg.addEventListener('click', () => setStep(Number(seg.dataset.seg) + 1)));
	prev?.addEventListener('click', () => setStep(engine.current - 1));
	next?.addEventListener('click', () => setStep(engine.current + 1));
	auto?.addEventListener('click', () => {
		if (playing) {
			playing = false;
			engine.stop();
			setAuto();
		} else {
			playAuto();
		}
	});

	figure.querySelector('.stepper-dots')?.addEventListener('keydown', (event) => {
		if (event.key !== 'ArrowLeft' && event.key !== 'ArrowRight') return;
		event.preventDefault();
		const n = Math.max(
			0,
			Math.min(engine.count, engine.current + (event.key === 'ArrowRight' ? 1 : -1))
		);
		setStep(n);
		if (n >= 1) dots[n - 1]?.focus();
	});

	onFirstView(figure, () => {
		if (reducedMotion()) {
			engine.set(engine.count);
			return;
		}
		playAuto();
	});
}

/** 播放按钮：从头播完，再点重播。 */
function bindPlay(figure) {
	const button = figure.querySelector('[data-play-button]');
	if (!button) return;
	const engine = createStepEngine(figure);
	if (!engine.count) return;

	figure.classList.add('is-armed');
	engine.render();

	const setLabel = (state) => {
		button.dataset.state = state;
		button.textContent = state === 'done' ? button.dataset.labelReplay : button.dataset.labelPlay;
	};
	setLabel('idle');

	const play = () => {
		setLabel('playing');
		engine.play(750, () => setLabel('done'));
	};

	button.addEventListener('click', play);
	onFirstView(figure, () => {
		if (reducedMotion()) {
			engine.set(engine.count);
			setLabel('done');
			return;
		}
		play();
	});
}

/** 扫描页面，给所有动图接上行为。多个组件各自调用也无妨。 */
export function initFigures() {
	document.querySelectorAll('[data-stepper]').forEach((figure) => {
		if (figure.dataset.figuresInit) return;
		figure.dataset.figuresInit = '1';
		bindStepper(figure);
	});
	document.querySelectorAll('[data-play]').forEach((figure) => {
		if (figure.dataset.figuresInit) return;
		figure.dataset.figuresInit = '1';
		bindPlay(figure);
	});
}
