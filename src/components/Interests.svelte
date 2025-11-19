<script>
	import { ChevronRight, Shapes } from "lucide-svelte";
</script>

<section class="interests">
	<h4>
		<div class="icon-container">
			<Shapes />
		</div>
		Interests
	</h4>

	<ul>
		<details>
			<summary class="link">
				<div class="chevron">
					<ChevronRight />
				</div>
				Programming
			</summary>

			<ul>
				<li>Front-end development (React, Web Components)</li>
				<li>Back-end development (Express, Fastify, Actix)</li>
				<li>Desktop development (GTK, Tauri, Web-based frameworks)</li>
			</ul>
		</details>

		<li>Web Browsers</li>

		<li>UI/UX Design</li>

		<li>Server Management</li>

		<li>Cybersecurity</li>
	</ul>
</section>

<style>
	ul li {
		margin-inline-start: 1.3rem;
		font-weight: 500;
	}

	summary {
		max-width: fit-content;
		align-items: center;
		margin-inline-start: -0.7rem;
		gap: 0.25rem;
	}

	summary .chevron {
		display: flex;
		align-items: center;
		transition: transform 0.1s ease-in-out;
	}

	details[open] .chevron {
		transform: rotate(90deg);
	}

	details ul {
		font-size: 0.75em;
		line-height: 1.75em;
	}
</style>
