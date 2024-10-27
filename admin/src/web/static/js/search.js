function search(term, redirect) {
    original_term = term.trim()
    term = original_term.toLowerCase()

    // Try to open destination directly, only display search page if multiple results
    if (packages.includes(term)) {
        window.open('/packages/'+encodeURIComponent(term)+'.html','_self');
        return false;
    }
    if (term in ids && ids[term].length == 1) {
        window.open('/advisories/'+encodeURIComponent(ids[term][0])+'.html','_self');
        return false;
    }

    // We can't redirect directly, so we'll display the results page.
    // For this we need to be on the search page, so let's redirect if not already there.
    if (redirect) {
        const params = new URLSearchParams({
            q: encodeURIComponent(original_term),
        });
        window.open('/search.html?'+params.toString(),'_self');
        return false;
    }

    // We can't redirect directly and are now on the search page: let's display search results
    // SECURITY: we need to escape the user-provided search term here to prevent reflected XSS.

    // use document.createTextNode for escaping
    document.getElementById('searched-term').innerHTML = ""
    document.getElementById('searched-term').appendChild(document.createTextNode("Search results for '"+original_term+"'"));
    if (term in ids) {
        var ul = document.createElement('ul');
        ids[term].forEach(function (item, index) {
            var li = document.createElement('li');
            var a = document.createElement('a');
            a.setAttribute('href', "/advisories/"+encodeURIComponent(item)+".html");
            a.appendChild(document.createTextNode(item));
            li.appendChild(a)
            ul.appendChild(li)
        });
        document.getElementById('search-result').innerHTML = ul.outerHTML;
    } else {
        document.getElementById('search-result').innerHTML = "<p>No results.</p>";
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
        search(decodeURIComponent(term), false)
    }
}
