<!-- Copyright (c) 2024 Kai Lawrence -->
<!--
Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.
-->

<script lang="ts">
	import { Tab, TabContent, Tabs } from 'carbon-components-svelte';
	import TopicTree from './topic_tree.svelte';
	import Messages from './messages.svelte';
	import PublishMessage from './publish_message.svelte';
	import type { AppState } from '$lib/state';
	import Pipeline from './pipeline.svelte';

	export let app: AppState;
	export let socket: WebSocket;

	let winWidth: number;
</script>

<svelte:window bind:innerWidth={winWidth} />

<div style="height: calc(100vh - 8em) !important; display: flex; flex-direction: column">
	<div style="display: flex">
		<div style="flex: 1; margin: 1em; min-width: 30em; max-width: 50em">
			<Tabs autoWidth type="container">
				<Tab label="Treeview" />
				<Tab label="Pipeline" />
				<svelte:fragment slot="content">
					<TabContent>
						<TopicTree bind:broker={app.brokerRepository[app.selectedBroker]} />
					</TabContent>
					<TabContent>
						<Pipeline
							bind:pipelines={app.pipelines}
							bind:broker={app.brokerRepository[app.selectedBroker]}
							bind:socket
						/>
					</TabContent>
				</svelte:fragment>
			</Tabs>
		</div>
		{#if winWidth > 1920}
			<div style="flex: 0.25" />
		{/if}
		<div style="flex: 1; margin: 1em; min-width: 30em; max-width: 60em">
			<Messages bind:broker={app.brokerRepository[app.selectedBroker]} />
		</div>
	</div>
	<div style="flex: 1;" />
	<PublishMessage
		bind:savedCommands={app.commands}
		bind:selectedBroker={app.selectedBroker}
		bind:socket
		bind:broker={app.brokerRepository[app.selectedBroker]}
	/>
</div>
