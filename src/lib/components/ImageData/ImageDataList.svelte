<script lang="ts">
	import ListComponent from "../common/ListComponent.svelte";
	import ImageDataListItem from "./ImageDataListItem.svelte";

    export let items
	export let handleDelete
</script>

<ListComponent items={items}  renderItem={ImageDataListItem}/>