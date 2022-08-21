import { Fab, Tooltip } from "@mui/material";
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
          onClick={() => setShowSideMenu(false)}
        >
          <Tooltip title="メニューを閉じる">
            <KeyboardDoubleArrowLeftIcon />
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
          onClick={() => setShowSideMenu(true)}
        >
          <Tooltip title="メニューを開く">
            <KeyboardDoubleArrowRightIcon />
          </Tooltip>
        </Fab>
      )}
    </React.Fragment>
  );
}

export default SideMenuToggle;
