import { useQuery } from "@tanstack/react-query";
import React, { Fragment } from "react";
import { useParams } from "react-router-dom";
import fetchDiary from "../api/fetchDiary";

function Diary() {
  const { diaryId } = useParams<{ diaryId: string }>();
  const diary = useQuery(["diary", diaryId!!], () => fetchDiary(diaryId!!));

  if (diary.isLoading) {
    return <>is loading...</>;
  } else if (diary.isError) {
    return <>なんかエラー</>;
  }
  return (
    <Fragment>
      <div dangerouslySetInnerHTML={{ __html: diary.data.html }} />
    </Fragment>
  );
}

export default Diary;
