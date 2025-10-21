function getUrlVars() {
    const vars = {};
    const query = window.location.search.substring(1);
    const pairs = query.split("&");
    for (let i = 0; i < pairs.length; i++) {
        const pair = pairs[i].split("=");
        vars[pair[0]] = decodeURIComponent(pair[1]);
    }
    return vars;
}

function showService() {
    const id = getUrlVars()["id"];

    fetch(`/api/v1/content/services/${id}`)
        .then(response => {
            if (!response.ok) {
                throw new Error("Network response was not ok");
            }
            return response.json();
        })
        .then(service => {
            // compile Handlebars template
            const source = document.getElementById("service-template").innerHTML;
            const template = Handlebars.compile(source);

            // render with data
            const html = template(service);

            // inject into DOM
            document.getElementById("serviceList").innerHTML = html;
        })
        .catch(error => {
            console.error("Fetch error:", error);
        });
}

document.addEventListener("DOMContentLoaded", showService);
