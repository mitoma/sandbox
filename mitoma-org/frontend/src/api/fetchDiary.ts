type DiaryResp = {
  html: string;
};

async function fetchDiary(diaryId: string): Promise<DiaryResp> {
  return fetch(`/api/v1/content/diary/${diaryId}`, {
    method: "GET",
  }).then((res) => res.json());
};

export default fetchDiary;
