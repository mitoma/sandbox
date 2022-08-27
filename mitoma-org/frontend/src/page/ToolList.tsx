import React from "react";
import { Link } from "react-router-dom";

function ToolList() {
  return (
    <React.Fragment>
      <h1>小物たち</h1>
      <Link to="/tool/calcAge" style={{ textDecoration: "none" }}>
        時間けいさん君
      </Link>
    </React.Fragment>
  );
}

export default ToolList;
