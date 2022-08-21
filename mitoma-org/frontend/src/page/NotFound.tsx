import { useQuery } from "@tanstack/react-query";
import React from "react";
import fetchHome from "../api/fetchHome";

function NotFound() {
  return (
    <React.Fragment>
      <h1>Not Found</h1>
    </React.Fragment>
  );
}

export default NotFound;
