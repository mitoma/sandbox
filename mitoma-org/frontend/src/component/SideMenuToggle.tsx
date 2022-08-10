import { Fab, IconButton, Tooltip } from "@mui/material";
import React from "react";
import KeyboardDoubleArrowLeftIcon from "@mui/icons-material/KeyboardDoubleArrowLeft";
import KeyboardDoubleArrowRightIcon from "@mui/icons-material/KeyboardDoubleArrowRight";

type SideMenuToggleProps = {
  defaultDrawerWidth: number;
  showSideMenu: boolean;
  setShowSideMenu: (value: boolean) => void;
};

function SideMenuToggle({
  defaultDrawerWidth,
  showSideMenu,
  setShowSideMenu,
}: SideMenuToggleProps) {
  return (
    <React.Fragment>
      {showSideMenu ? (
        <Fab
          size="small"
          sx={{
            position: "absolute",
            top: 16,
            left: defaultDrawerWidth - 16,
            boxShadow: 0,
            zIndex: (theme) => theme.zIndex.drawer + 1,
          }}
        >
          <Tooltip title="メニューを閉じる">
            <IconButton onClick={() => setShowSideMenu(false)}>
              <KeyboardDoubleArrowLeftIcon />
            </IconButton>
          </Tooltip>
        </Fab>
      ) : (
        <Fab
          size="small"
          sx={{
            position: "absolute",
            top: 16,
            left: 16,
            boxShadow: 0,
          }}
        >
          <Tooltip title="メニューを開く">
            <IconButton onClick={() => setShowSideMenu(true)}>
              <KeyboardDoubleArrowRightIcon />
            </IconButton>
          </Tooltip>
        </Fab>
      )}
    </React.Fragment>
  );
}

export default SideMenuToggle;
