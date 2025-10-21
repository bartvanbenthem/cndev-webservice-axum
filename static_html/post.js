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

function showPost() {
    const id = getUrlVars()["id"];

    fetch(`/api/v1/content/posts/${id}`)
        .then(response => {
            if (!response.ok) {
                throw new Error("Network response was not ok");
            }
            return response.json();
        })
        .then(post => {
            // compile Handlebars template
            const source = document.getElementById("post-template").innerHTML;
            const template = Handlebars.compile(source);

            // render template with post data
            const html = template(post);

            // inject into DOM
            document.getElementById("postList").innerHTML = html;
        })
        .catch(error => {
            console.error("Fetch error:", error);
        });
}

document.addEventListener("DOMContentLoaded", showPost);
