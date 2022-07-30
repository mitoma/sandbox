import React, { useState, useEffect, Fragment } from "react";

// TODO SWR に置き換えよう
function FetchMarkdown() {
  const [markdownHtml, setMarkdownHtml] = useState({ __html: "<b>Loading...</b>" });

  useEffect(() => {
    fetch("/api/v1/health", { method: "GET" })
      .then((res) => res.json())
      .then((data) => {
        console.log(data);
        setMarkdownHtml(data);
      });
  }, []);

  return (
    <Fragment>
      <div dangerouslySetInnerHTML={markdownHtml} />
    </Fragment>
  );
}

export default FetchMarkdown;
