type AboutMeResp = {
  html: string;
};

async function fetchAboutMe(): Promise<AboutMeResp> {
  return fetch("/api/v1/content/about_me", { method: "GET" }).then((res) => res.json());
}

export default fetchAboutMe;
