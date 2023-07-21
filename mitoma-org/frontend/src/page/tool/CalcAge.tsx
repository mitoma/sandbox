import {
  Button,
  Card,
  CardActions,
  CardContent,
  CardHeader,
  IconButton,
  TextField,
} from "@mui/material";
import React, { useState } from "react";
import {
  dateToString,
  dateToJapaneseFormatString,
  intervalString,
  parseForLocalStorage,
  formatForLocalStorage,
  immutableSwap,
} from "./calc";
import ArrowUpwardIcon from "@mui/icons-material/ArrowUpward";
import ArrowDownwardIcon from "@mui/icons-material/ArrowDownward";
import DeleteIcon from "@mui/icons-material/Delete";
import AddCircleIcon from "@mui/icons-material/AddCircle";
import useLocalStorage from "../../hook/useLocalStorage";

type SpecialDayForView = {
  name: string;
  date: string;
  eraDate: string;
  age: string;
};

type SpecialDay = {
  name: string;
  date: string;
};

function createSpecialDayForView(
  name: string,
  date: Date,
  currentDate: Date
): SpecialDayForView {
  return {
    name,
    date: dateToString(date),
    eraDate: dateToJapaneseFormatString(date),
    age: intervalString(date, currentDate),
  };
}

function CalcAge() {
  const date: Date = new Date();
  const [storedSpecialDays, setStoredSpecialDays] = useLocalStorage<
    SpecialDay[]
  >("specialDays", []);
  const [specialDayName, setSpecialDayName] = useState<string>("");
  const [specialDayDate, setSpecialDayDate] = useState<string>("1980-01-01");
  const specialDay = { name: specialDayName, date: specialDayDate };

  const views = storedSpecialDays.map((storedRow) =>
    createSpecialDayForView(
      storedRow.name,
      parseForLocalStorage(storedRow.date),
      date
    )
  );

  return (
    <React.Fragment>
      <h1>時間けいさん君</h1>
      作ってる途中
      {views.map((row, index) => (
        <Card sx={{ margin: 2 }} id={row.name}>
          <CardHeader title={row.name} />
          <CardContent>
            <p>{row.date}</p>
            <p>{row.eraDate}</p>
            <p>{row.age}</p>
          </CardContent>
          <CardActions>
            <IconButton
              onClick={() => {
                setStoredSpecialDays(
                  immutableSwap(storedSpecialDays, index - 1, index)
                );
              }}
            >
              <ArrowUpwardIcon />
            </IconButton>
            <IconButton
              onClick={() => {
                setStoredSpecialDays(
                  immutableSwap(storedSpecialDays, index, index + 1)
                );
              }}
            >
              <ArrowDownwardIcon />
            </IconButton>
            <Button
              startIcon={<DeleteIcon />}
              onClick={() => {
                if (!window.confirm("本当に削除する？")) {
                  return;
                }
                setStoredSpecialDays(
                  storedSpecialDays.filter((_, idx) => idx !== index)
                );
              }}
            >
              削除
            </Button>
          </CardActions>
        </Card>
      ))}
      <Card sx={{ margin: 2 }}>
        <CardHeader title="日を足す" />
        <CardContent>
          <TextField
            id="dayName"
            label="名前"
            variant="outlined"
            value={specialDayName}
            onChange={(event) => setSpecialDayName(event.target.value)}
          />
          <TextField
            id="date"
            label="日付"
            type="date"
            defaultValue={formatForLocalStorage(date)}
            onChange={(event) => setSpecialDayDate(event.target.value)}
            InputLabelProps={{
              shrink: true,
            }}
          />
        </CardContent>
        <CardActions>
          <Button
            startIcon={<AddCircleIcon />}
            variant="contained"
            color="secondary"
            onClick={() => {
              setStoredSpecialDays(
                storedSpecialDays
                  .filter((day) => day.name !== specialDayName)
                  .concat([specialDay])
              );
            }}
          >
            追加
          </Button>
        </CardActions>
      </Card>
    </React.Fragment>
  );
}

export default CalcAge;
