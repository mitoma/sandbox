type Entry = {
  title: string;
  path: string;
};

type BlogListResp = {
  path: string;
  list: Array<Entry>;
};

async function fetchBlogList(): Promise<BlogListResp> {
  return fetch(`/api/v1/content/blog:list_md`, {
    method: "GET",
  }).then((res) => res.json());
}

export default fetchBlogList;
