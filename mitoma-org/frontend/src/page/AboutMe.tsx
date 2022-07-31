import { useQuery } from "@tanstack/react-query";
import React from "react";
import fetchAboutMe from "../api/fetchAboutMe";

function AboutMe() {
  const aboutMe = useQuery(["aboutMe"], fetchAboutMe);

  if (aboutMe.isLoading) {
    return <>is loading...</>;
  } else if (aboutMe.isError) {
    return <>なんかエラー</>;
  }
  return (
    <React.Fragment>
      <div dangerouslySetInnerHTML={{ __html: aboutMe.data.html }} />
    </React.Fragment>
  );
}

export default AboutMe;
