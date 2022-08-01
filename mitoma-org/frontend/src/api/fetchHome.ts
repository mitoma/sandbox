type HomeResp = {
  html: string;
};

async function fetchHome(): Promise<HomeResp> {
  return fetch("/api/v1/content/home", { method: "GET" }).then((res) =>
    res.json()
  );
}

export default fetchHome;
