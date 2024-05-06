<style>
    #page{
        display: flex;
    }
    #side_nav{
        width: 25%;
    }

    #side_nav a{
        display: block;
    }
    
</style>

<div id="page">
    <div id="side_nav">

        <a href="/">Machines</a>
        <a href="/repository">Repository</a>
    </div>
    <div>
        <slot></slot>
    </div>
</div>