type DiaryResp = {
  html: string;
};

async function fetchAboutMe(): Promise<DiaryResp> {
  return fetch("/api/v1/content/diary/2022-08-01", { method: "GET" }).then((res) => res.json());
}

export default fetchAboutMe;
