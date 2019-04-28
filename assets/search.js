const searchbar = document.getElementById("search")
const noPreperation = document.getElementById("noPreperation")
const noMessageLimit = document.getElementById("noMessageLimit")

function enableInputs() {
    searchbar.disabled = false
    noPreperation.disabled = false
    noMessageLimit.disabled = false

    searchbar.onkeyup = () => { run() }
    noPreperation.onchange = () => { run() }
    noMessageLimit.onchange = () => { run() }
}

let index = []

function populateIndex() {
    let newIndex = []
    const list = document.getElementById("provider_list")

    const items = list.childNodes;
    for (let i = items.length; i--;) {
        const elem = items[i];
        if (!elem.dataset) continue;
        newIndex.push({
            elem: elem,
            searchkey: elem.dataset.searchkey,
            preparation: elem.dataset.preparation === "true",
            messagelimit: elem.dataset.messagelimit,
        })
    }

    index = newIndex

}

function update(mask) {
    for (let i = 0; i < index.length; i++) {
        if(index[i].elem.hidden === !mask[i]) continue;
        index[i].elem.hidden = !mask[i]
    }
}

function search(searchword) {
    return index.map(({ searchkey }) => searchkey.indexOf(searchword) !== -1)
}

function filter(property, value) {
    return index.map(({ [property]:p }) => p === value)
}

async function runSearch() {
    if (index.length === 0) return
    let masks = []

    if (searchbar.value !== "")
        masks.push(search(searchbar.value))
    if (noPreperation.checked)
        masks.push(filter('preparation', false))
    if (noMessageLimit.checked)
        masks.push(filter('messagelimit', "Unlimited"))

    let search_result = []
    
    // apply all masks on top of eachother ( if one false - all false)
    for (let i = 0; i < index.length; i++) {
        let res = true;
        for (let j = 0; j < masks.length; j++) {
            const thing = masks[j][i];
            if (masks[j][i]) {
                continue
            } else {
                res = false
            }
        }
        search_result.push(res)
    }

    update(search_result)
}

let is_running = false
let scheduled = false
const run = async () => { // this debounce is probably not nessesary
    if (!is_running) {
        is_running = true
        await runSearch(searchbar.value)
        is_running = false
        if (scheduled) {
            scheduled = false
            await run()
        }
    } else {
        scheduled = true
    }
}

(function () {
    console.time('init')
    populateIndex()

    enableInputs()

    console.timeEnd('init')
})()