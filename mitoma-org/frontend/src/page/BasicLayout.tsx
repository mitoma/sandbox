import { Box, Container, Toolbar } from "@mui/material";
import { useState } from "react";
import { Outlet } from "react-router-dom";
import ContentSpacer from "../component/ContentSpacer";
import SideMenu from "../component/SideMenu";

function BasicLayout() {
  const [showSideMenu, setShowSideMenu] = useState(false);

  return (
    <Box sx={{ display: "flex" }}>
      <SideMenu showSideMenu={showSideMenu} setShowSideMenu={setShowSideMenu} />
      <Container maxWidth="md" component="main">
        <Toolbar />
        <ContentSpacer
          showSideMenu={showSideMenu}
          setShowSideMenu={setShowSideMenu}
        />
        <Outlet />
      </Container>
    </Box>
  );
}

export default BasicLayout;
