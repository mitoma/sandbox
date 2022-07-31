type AboutMeResp = {
  __html: string;
};

async function fetchAboutMe(): Promise<AboutMeResp> {
  return fetch("/api/v1/health", { method: "GET" }).then((res) => res.json());
}

export default fetchAboutMe;
