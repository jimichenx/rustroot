import '@expressive-code/core';

declare module '@expressive-code/core' {
	interface ExpressiveCodeBlockProps {
		/**
		 * 折叠前保留的行数。代码不超过这个行数时不显示按钮。
		 * Markdown 围栏写在语言标记后面：`collapse=24`。
		 */
		collapse?: number;
	}
}
