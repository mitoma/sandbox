type BlogResp = {
  html: string;
};

async function fetchBlog(blogPath: string): Promise<BlogResp> {
  return fetch(`/api/v1/content/blog/${blogPath}`, {
    method: "GET",
  }).then((res) => res.json());
}

export default fetchBlog;
