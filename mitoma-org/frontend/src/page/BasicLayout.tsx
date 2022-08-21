import { Box, Container } from "@mui/material";
import { Outlet } from "react-router-dom";
import ContentSpacer from "../component/ContentSpacer";
import SideMenu from "../component/SideMenu";
import useLocalStorage from "../hook/useLocalStorage";

function BasicLayout() {
  const [showSideMenu, setShowSideMenu] = useLocalStorage("showSideMenu", true);

  return (
    <Box sx={{ display: "flex" }}>
      <SideMenu showSideMenu={showSideMenu} setShowSideMenu={setShowSideMenu} />
      <Container maxWidth="md" component="main">
        <ContentSpacer showSideMenu={showSideMenu} />
        <Outlet />
      </Container>
    </Box>
  );
}

export default BasicLayout;
