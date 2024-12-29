<style>
    #results{
        height:150px;
    }
    
    table {
        width:100%;
        border: 1px solid grey
    }
    
    table th, td{
        width: 5%;
        border:1px solid grey
    }
</style>
<script lang="ts">
    
    import { UpDownArrow } from "$lib";

    type TableHeader = "epoch"|"train_acc"|"test_acc"|"train_loss"|"test_loss"
    interface TableSortOption{
        column: TableHeader,
        asc: boolean
    }

    $: tableSortOption  = {} as TableSortOption

    $: isSortedAsc = (header:TableHeader): boolean => {
        return tableSortOption.column === header && tableSortOption.asc
    }
    
    function handleSetTableSort(header:TableHeader){
        tableSortOption = {column:header, asc:tableSortOption.asc ? !tableSortOption.asc : true}
        console.log("set sort", tableSortOption)
    }

    function sortSeries(a:TrainingDataDataSeries, b:TrainingDataDataSeries): number{
        
        if (a.epoch < b.epoch){
            return -1
        }
        return 1        
    }
</script>
<div id="results" class="w-100">
    <h4>Training Results</h4>
    <div class="y-scrollable">
        <table>
            <thead>
                <tr>
                    <th scope="col">Epoch <UpDownArrow up={isSortedAsc("epoch")} handleClick={()=>handleSetTableSort("epoch")}/> </th>
                    <th scope="col">Train Accuracy <UpDownArrow up={isSortedAsc("train_acc")} handleClick={()=>handleSetTableSort("train_acc")}/></th>
                    <th scope="col">Test Accuracy <UpDownArrow up={isSortedAsc("test_acc")} handleClick={()=>handleSetTableSort("test_acc")}/></th>
                    <th scope="col">Train Validation <UpDownArrow up={isSortedAsc("train_loss")} handleClick={()=>handleSetTableSort("train_loss")}/></th>
                    <th scope="col">Test Validation <UpDownArrow up={isSortedAsc("test_loss")} handleClick={()=>handleSetTableSort("test_loss")}/></th>
                </tr>
            </thead>
            <tbody>
                {#each trainingResults.sort(sortSeries) as result}
                <tr>
                    <td>{result.trainData.epoch}</td>
                    <td>{result.trainData.train_acc}</td>
                    <td>{result.trainData.test_acc}</td>
                    <td>{result.trainData.train_loss.toFixed(3)}</td>
                    <td>{result.trainData.val_loss.toFixed(3)}</td>
                </tr>
                {/each}
            </tbody>
        </table>
        
    </div>
</div>
