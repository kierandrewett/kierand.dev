<script lang="ts">
	import { onMount } from "svelte";

	let rotateHours = 0;
	let rotateMinutes = 0;
	let rotateSeconds = 0;
	let dateString: string | null = null;

	function updateHands() {
		const date = new Date();

		rotateHours = 30 * (date.getHours() - 12) + date.getMinutes() / 2;
		rotateMinutes = 6 * date.getMinutes();
		rotateSeconds = 6 * date.getSeconds();
	}

	function createDateString() {
		const date = new Date();

		const parts = new Intl.DateTimeFormat("en-GB", {
			day: "numeric",
			month: "long",
			year: "numeric",
			hour: "2-digit",
			minute: "2-digit",
			hour12: false
		}).formatToParts(date);

		const day = parts.find((p) => p.type === "day")?.value;
		const month = parts.find((p) => p.type === "month")?.value;
		const year = parts.find((p) => p.type === "year")?.value;
		const hour = parts.find((p) => p.type === "hour")?.value;
		const minute = parts.find((p) => p.type === "minute")?.value;

		const d = Number(day);
		const suffix =
			d % 10 === 1 && d !== 11
				? "st"
				: d % 10 === 2 && d !== 12
				? "nd"
				: d % 10 === 3 && d !== 13
				? "rd"
				: "th";

		dateString = `${day}${suffix} ${month} ${year} at ${hour}:${minute}`;
	}

	onMount(() => {
		createDateString();

		updateHands();
		setInterval(updateHands, 1000);
	});
</script>

<div class="clock">
	<div class="clock-analogue">
		<div class="clock-dot" />
		<div class="clock-hand hours" style="--rotate: {rotateHours}deg" />
		<div class="clock-hand minutes" style="--rotate: {rotateMinutes}deg" />
		<div class="clock-hand seconds" style="--rotate: {rotateSeconds}deg" />
	</div>
	<span>
		{#if dateString}
			{dateString}
		{:else}
			—
		{/if}
	</span>
</div>

<style>
	.clock {
		display: flex;
		gap: 0.75rem;
		align-items: center;
		font-weight: 500;
		color: var(--pink-lighter);
	}

	.clock .clock-analogue {
		width: 20px;
		height: 20px;
		border: 2px solid currentColor;
		border-radius: 20px;
		position: relative;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.clock .clock-analogue > * {
		box-shadow: 0 1.6px 3.6px 0 rgba(0, 0, 0, 0.132), 0 0.3px 0.9px 0 rgba(0, 0, 0, 0.108);
	}

	.clock .clock-dot {
		width: 3px;
		height: 3px;
		position: absolute;
		background-color: currentColor;
		border-radius: 3px;
		z-index: 2;
	}

	.clock .clock-hand {
		width: 2px;
		height: 7px;
		top: 12%;
		left: 44%;
		position: absolute;
		background-color: currentcolor;
		transform-origin: center bottom 0px;
		transform: rotate(var(--rotate, 0deg));
		border-radius: 100px;
		transition: 2s transform ease-in-out;
	}

	.clock .clock-hand.seconds {
		background-color: rgb(250, 146, 146);
		z-index: 0;
		transition: transform 0s ease-out 0s;
	}

	.clock span {
		font-size: 1rem;
	}
</style>
