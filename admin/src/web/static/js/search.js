function search(term, redirect) {
    // Try to open destination directly, only display search page if multiple results
    package = term.trim()
    if (packages.includes(package)) {
        window.open('/packages/'+package+'.html','_self');
        return false;
    }
    id = term.trim()
    if (id in ids && ids[id].length == 1) {
        window.open('/advisories/'+ids[id][0]+'.html','_self');
        return false;
    }

    // If not already on search page, let's redirect (and keep search term)
    if (redirect) {
        const params = new URLSearchParams({
            q: term,
        });
        window.open('/search.html?'+params.toString(),'_self');
        return false;
    }

    // Display search results
    if (id in ids) {
        displayResult = ""
        ids[id].forEach(function (item, index) {
            console.log(item, index);
            displayResult = displayResult.concat("<li><a href=/advisories/"+item+".html>"+item+"</a></li>")
        });
        document.getElementById('search-result').innerHTML = "<ul>"+displayResult+"</ul>";
        document.getElementById('searched-term').innerHTML = "Search results for '"+id+"'";
    } else {
        document.getElementById('search-result').innerHTML = "<p>No results.</p>";
        document.getElementById('searched-term').innerHTML = "Search results for '"+term+"'";
    }
}

function searchform() {
    var term = document.getElementById('search-term').value
    search(term, true)
    // Don't submit form with default behavior
    return false;
}

function searchformindex() {
    var term = document.getElementById('search-term-index').value
    search(term, true)
    // Don't submit form with default behavior
    return false;
}

if (window.location.pathname.endsWith("search.html")) {
    const term = new URLSearchParams(window.location.search).get("q");
    if (term != null){
        search(term, false)
    }
}
