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
    #content{
        width:75%;
    }
    
</style>

<div id="page">
    <div id="side_nav">

        <a href="/">Machines</a>
        <a href="/projects">Projects</a>
    </div>
    <div id="content">
        <slot></slot>
    </div>
</div>