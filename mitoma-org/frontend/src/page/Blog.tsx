import { useQuery } from "@tanstack/react-query";
import { Fragment } from "react";
import { useParams } from "react-router-dom";
import fetchBlog from "../api/fetchBlog";

function Blog() {
  const { blogPath } = useParams<{ blogPath: string }>();
  const diary = useQuery(["blog", blogPath!!], () => fetchBlog(blogPath!!));

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

export default Blog;
