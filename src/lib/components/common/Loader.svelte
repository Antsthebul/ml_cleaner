<style>
    .loading{
        position:fixed;
        top:0;
        width: 100%;
        background-color:rgba(255, 255, 255, 0.566);
    }
    .inner-loader{
        width: 100%;
        z-index: 9999;
        height:100vh;
        font-size: 4em;
        margin: 0 auto;
        color:rgba(6, 115, 205, 0.884);
        display:flex;
        justify-content: center;
        align-items: center;
    }    
</style>
<script lang="ts">
    import LineMdLoadingLoop from '~icons/line-md/loading-loop';

</script>

<div class="loading">
    <div class="inner-loader">

        <LineMdLoadingLoop />
    </div>
</div>