<style lang="scss">
    #page{
        display: flex;
        height: 100vh;
    }
    

</style>
<script lang="ts">
    import '../app.css';
    import { SideNav } from '$lib';

</script>

<div id="page">
    <SideNav/>
    <div id="app" class="w-100">
        <slot></slot>
    </div>
</div>