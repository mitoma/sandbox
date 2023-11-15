import { useQuery } from "@tanstack/react-query";
import { Fragment } from "react";
import { useParams } from "react-router-dom";
import fetchBlog from "../api/fetchBlog";

function Blog() {
  const { blogPath } = useParams<{ blogPath: string }>();
  const fetchThisBlobArticle = () => fetchBlog(blogPath ?? "");
  const diary = useQuery({
    queryKey: ["blog", blogPath ?? ""],
    queryFn: fetchThisBlobArticle,
  });

  if (diary.isLoading || diary.data === undefined) {
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
