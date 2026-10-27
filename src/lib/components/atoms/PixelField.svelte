<script lang="ts">
	import { onMount } from 'svelte';

	/** Cell edge in CSS px; one monospace glyph per cell. */
	const CELL = 14;
	/** Pointer influence radius in cells. */
	const RADIUS = 5;
	const GLYPHS = '01{}[]<>()/\\|=+-*#$%&;:~^?!_';
	const randomGlyph = () => GLYPHS[Math.floor(Math.random() * GLYPHS.length)];
	const DECAY = 0.93;

	let canvas: HTMLCanvasElement;

	onMount(() => {
		if (matchMedia('(prefers-reduced-motion: reduce)').matches) return;

		const ctx = canvas.getContext('2d');
		if (!ctx) return;

		let cols = 0;
		let rows = 0;
		let width = 0;
		let height = 0;
		let heat = new Float32Array(0);
		let glyphs: string[] = [];
		let font = '12px monospace';
		let frame = 0;
		let color = '157 111 212';

		const resize = () => {
			const dpr = window.devicePixelRatio || 1;
			width = window.innerWidth;
			height = window.innerHeight * 0.85;
			cols = Math.ceil(width / CELL);
			rows = Math.ceil(height / CELL);
			canvas.width = Math.ceil(width * dpr);
			canvas.height = Math.ceil(height * dpr);
			ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
			heat = new Float32Array(cols * rows);
			glyphs = Array.from({ length: cols * rows }, randomGlyph);
			const styles = getComputedStyle(canvas);
			font = `${CELL - 2}px ${styles.getPropertyValue('--font-mono').trim() || 'monospace'}`;
			color = styles
				.getPropertyValue('--color-accent')
				.trim()
				.replace(/^#/, '')
				.replace(/(..)(..)(..)/, (_, r, g, b) => [r, g, b].map((h) => parseInt(h, 16)).join(' '));
		};

		const draw = () => {
			ctx.clearRect(0, 0, width, height);
			ctx.font = font;
			ctx.textAlign = 'center';
			ctx.textBaseline = 'middle';
			let alive = false;

			for (let i = 0; i < heat.length; i++) {
				const v = heat[i];
				if (v < 0.02) {
					heat[i] = 0;
					continue;
				}
				alive = true;
				// flicker: hot cells swap glyphs now and then
				if (Math.random() < 0.12) glyphs[i] = randomGlyph();
				ctx.fillStyle = `rgb(${color} / ${Math.min(1, v * 0.9)})`;
				ctx.fillText(
					glyphs[i],
					(i % cols) * CELL + CELL / 2,
					Math.floor(i / cols) * CELL + CELL / 2
				);
				heat[i] = v * DECAY;
			}

			frame = alive ? requestAnimationFrame(draw) : 0;
		};

		const onMove = (e: PointerEvent) => {
			const cx = Math.floor(e.clientX / CELL);
			const cy = Math.floor(e.clientY / CELL);

			for (let dy = -RADIUS; dy <= RADIUS; dy++) {
				for (let dx = -RADIUS; dx <= RADIUS; dx++) {
					const x = cx + dx;
					const y = cy + dy;
					if (x < 0 || y < 0 || x >= cols || y >= rows) continue;

					const falloff = 1 - Math.hypot(dx, dy) / RADIUS;
					if (falloff <= 0) continue;

					const i = y * cols + x;
					heat[i] = Math.max(heat[i], falloff * (0.4 + Math.random() * 0.6));
				}
			}

			if (!frame) frame = requestAnimationFrame(draw);
		};

		resize();
		window.addEventListener('resize', resize);
		window.addEventListener('pointermove', onMove, { passive: true });

		return () => {
			cancelAnimationFrame(frame);
			window.removeEventListener('resize', resize);
			window.removeEventListener('pointermove', onMove);
		};
	});
</script>

<canvas
	bind:this={canvas}
	aria-hidden="true"
	class="pointer-events-none fixed inset-x-0 top-0 z-0 block h-[85vh] w-full [mask-image:linear-gradient(to_bottom,#000_65%,transparent)] select-none"
></canvas>
