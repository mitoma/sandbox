import { useQuery } from "@tanstack/react-query";
import React from "react";
import fetchHome from "../api/fetchHome";

function AboutMe() {
  const aboutMe = useQuery({ queryKey: ["aboutMe"], queryFn: fetchHome });

  if (aboutMe.isLoading || aboutMe.data === undefined) {
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
