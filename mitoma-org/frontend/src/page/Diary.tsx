import { useQuery } from "@tanstack/react-query";
import React, { Fragment } from "react";
import fetchDiary from "../api/fetchDiary";

function Diary() {
  const diary = useQuery(["diary"], fetchDiary);

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
