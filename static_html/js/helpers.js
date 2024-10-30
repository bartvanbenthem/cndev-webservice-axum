
function truncateContent(content, wordCount) {
    return content.split(" ").slice(0, wordCount).join(" ") + (content.split(" ").length > wordCount ? '...' : '');
  }