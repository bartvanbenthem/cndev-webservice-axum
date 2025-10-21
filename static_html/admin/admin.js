const token = localStorage.getItem("token");

document.addEventListener("DOMContentLoaded", () => {
    // Check if the token exists when the page is loaded.
    if (!token) {
        alert("You must be logged in to access this page.");
        window.location.href = "index.html";
        return;
    }

    loadAll();

    // Logout button functionality
    document.getElementById("logoutBtn").addEventListener("click", () => {
        // Remove token from localStorage
        localStorage.removeItem("token");

        // Redirect to index.html after logging out
        window.location.href = "index.html";
    });
});

function loadAll() {
    loadAdmins();
    loadPosts();
    bindAddPost();
}

function loadAdmins() {
    fetch("/api/v1/auth/users", {
        method: "GET",
        headers: {
            "Content-Type": "application/json",
            "Token": token
        }
    })
    .then(res => res.json())
    .then(data => {
        let table = `<table class='table table-striped'>
            <thead><tr><th>Username</th></tr></thead><tbody>`;
        data.forEach(user => {
            table += `<tr><td>${user.username}</td></tr>`;
        });
        table += `</tbody></table>`;
        document.getElementById("admins").innerHTML = table;
    })
    .catch(err => console.error("Error loading admins:", err));
}

function loadPosts() {
    fetch("/api/v1/content/posts")
    .then(res => res.json())
    .then(data => {
        let postList = `<table class='table table-striped'>
            <thead><tr><th>Title</th><th>Author</th><th>Content</th><th></th></tr></thead><tbody>`;
        data.forEach(post => {
            const url = `post-edit.html?id=${post.id}`;
            postList += `<tr id="post-${post.id}">
                <td><a href='${url}'>${post.title}</a></td>
                <td>${post.author}</td>
                <td>${truncateContent(post.content, 35)}</td>
                <td><a href="#" onclick="deletePost(${post.id})">Delete</a></td>
            </tr>`;
        });
        postList += `</tbody></table>`;
        document.getElementById("posts").innerHTML = postList;
    })
    .catch(err => console.error("Error loading posts:", err));
}

function bindAddPost() {
    document.getElementById("btnAddPost").addEventListener("click", () => {
        const title = document.getElementById("newPostName").value.trim();
        const author = document.getElementById("newPostAuthor").value.trim();
        const img = document.getElementById("newPostImage").value.trim();
        const content = document.getElementById("newPostContent").value.trim();

        if (!title || !author || !img || !content) {
            alert("You must enter a title, author, image and content of the post.");
            return;
        }

        const payload = { id: 0, title, author, img, content };

        fetch("/api/v1/content/posts/add", {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
                "Token": token
            },
            body: JSON.stringify(payload)
        })
        .then(res => {
            if (res.ok) {
                // If the status is 200 OK, reload posts
                loadPosts();
                
                // Reset the input fields after a successful post
                document.getElementById("newPostName").value = '';
                document.getElementById("newPostAuthor").value = '';
                document.getElementById("newPostImage").value = '';
                document.getElementById("newPostContent").value = '';
            } else {
                // If the response is not OK, handle the error
                res.text().then(text => {
                    console.error("Failed to add post:", text);
                    alert("Error adding post: " + text);
                });
            }
        })
        .catch(err => {
            console.error("Network or script error:", err);
            alert("An unexpected error occurred. See console for details.");
        });
    });
}

function deletePost(id) {
    if (!confirm("Are you sure you want to delete this post?")) return;

    fetch(`/api/v1/content/posts/delete/${id}`, {
        method: "DELETE", // Use DELETE method for the backend to handle
        headers: {
            "Content-Type": "application/json",
            "Token": token // The token for authorization
        }
    })
    .then(res => {
        if (res.ok) {
            // If the status is 200 OK, reload posts
            loadPosts();
        } else {
            // If the response is not OK, handle the error
            res.text().then(text => {
                console.error("Failed to delete post:", text);
                alert("Error deleting post: " + text);
            });
        }
    })
    .catch(err => {
        console.error("Error deleting post:", err);
        alert("An error occurred while deleting the post");
    });
}

function truncateContent(content, maxLength) {
    if (content.length > maxLength) {
        return content.substring(0, maxLength) + "...";
    }
    return content;
}