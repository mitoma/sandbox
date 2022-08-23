import {
  AppBar,
  Box,
  IconButton,
  Toolbar,
  Tooltip,
  Typography,
} from "@mui/material";
import MenuIcon from "@mui/icons-material/Menu";
import MenuOpenIcon from "@mui/icons-material/MenuOpen";

type ContentSpacerProps = {
  showSideMenu: boolean;
  setShowSideMenu: (value: boolean) => void;
};

function ContentSpacer({ showSideMenu, setShowSideMenu }: ContentSpacerProps) {
  return (
    <Box sx={{ flexGrow: 1 }}>
      <AppBar sx={{ boxShadow: 1, zIndex: (theme) => theme.zIndex.drawer + 1 }}>
        <Toolbar>
          {showSideMenu ? (
            <>
              <Tooltip title="メニューを閉じる">
                <IconButton onClick={() => setShowSideMenu(false)}>
                  <MenuOpenIcon />
                </IconButton>
              </Tooltip>
            </>
          ) : (
            <>
              {" "}
              <Tooltip title="メニューを開く">
                <IconButton onClick={() => setShowSideMenu(true)}>
                  <MenuIcon />
                </IconButton>
              </Tooltip>
            </>
          )}
          <img src="/logo.png" width="50" alt="profile icon" />
          <Box sx={{ padding: 2 }}>
            <Typography variant="h6">mitoma.org</Typography>
          </Box>
        </Toolbar>
      </AppBar>
    </Box>
  );
}

export default ContentSpacer;
