<style lang="scss">
    #page{
        display: flex;
    }

    
    #content{
        width:75%;
    }

</style>
<script lang="ts">
    import '../app.css';
    import Nav from '../components/Nav.svelte';

</script>

<div id="page">
    <Nav/>
    <div id="content">
        <slot></slot>
    </div>
</div>