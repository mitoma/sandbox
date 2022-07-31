import { useQuery } from "@tanstack/react-query";
import React, { Fragment } from "react";
import fetchAboutMe from "./api/fetchAboutMe";

function FetchMarkdown() {
  const aboutMe = useQuery(["aboutMe"], fetchAboutMe);

  if (aboutMe.isLoading) {
    return <>is loading...</>;
  } else if (aboutMe.isError) {
    return <>なんかエラー</>;
  }
  return (
    <Fragment>
      <div dangerouslySetInnerHTML={aboutMe.data} />
    </Fragment>
  );
}

export default FetchMarkdown;
