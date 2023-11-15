import { Link } from "react-router-dom";
import { useQuery } from "@tanstack/react-query";
import { Fragment } from "react";
import fetchBlogList from "../api/fetchBlogList";
import { Typography } from "@mui/material";

function BlogList() {
  const blog = useQuery({
    queryKey: ["blogList"],
    queryFn: fetchBlogList,
  });

  if (blog.isLoading || blog.data === undefined) {
    return <>is loading...</>;
  } else if (blog.isError) {
    return <>なんかエラー</>;
  }

  return (
    <Fragment>
      <h1>blogエントリ一覧</h1>
      {blog.data.list.map((entry) => {
        return (
          <Link to={`/${blog.data.path}/${entry.path}`}>
            <Typography variant="h6">{entry.title}</Typography>
          </Link>
        );
      })}
    </Fragment>
  );
}

export default BlogList;
