document.addEventListener("DOMContentLoaded", () => {
    const token = localStorage.getItem("token");

    if (!token) {
        alert("You must be logged in to access this page.");
        window.location.href = "index.html";
        return;
    }

    const urlParams = new URLSearchParams(window.location.search);
    const id = urlParams.get("id");

    fetch(`/api/v1/content/posts/${id}`)
        .then(response => response.json())
        .then(data => {
            document.getElementById("PostName").value = data.title;
            document.getElementById("PostAuthor").value = data.author;
            document.getElementById("PostImage").value = data.img;
            document.getElementById("PostContent").value = data.content;
        })
        .catch(error => console.error("Failed to load post:", error));

    document.getElementById("btnAddPost").addEventListener("click", () => {
        const title = document.getElementById("PostName").value.trim();
        const author = document.getElementById("PostAuthor").value.trim();
        const img = document.getElementById("PostImage").value.trim();
        const content = document.getElementById("PostContent").value.trim();

        if (!title || !author || !img || !content) {
            alert("All fields are required.");
            return;
        }

        fetch(`/api/v1/content/posts/update/${id}`, {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
                "Token": token
            },
            body: JSON.stringify({
                id: parseInt(id),
                title: title,
                author: author,
                img: img,
                content: content
            })
        })
        .then(response => {
            if (!response.ok) {
                throw new Error("Network response was not ok");
            }
            window.location.href = "/admin/admin.html";
        })
        .catch(error => {
            console.error("Failed to update post:", error);
            alert("An error occurred while updating the post.");
        });

    });
});