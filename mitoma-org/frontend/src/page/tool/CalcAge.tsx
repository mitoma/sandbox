import {
  Button,
  Paper,
  Table,
  TableBody,
  TableCell,
  TableContainer,
  TableHead,
  TableRow,
} from "@mui/material";
import React from "react";

function dateToJapaneseFormatString(date: Date): string {
  /*
  const parts = new Intl.DateTimeFormat("ja-JP-u-ca-japanese", {
    era: "long",
  }).formatToParts(date);
  const era = parts.find((obj) => obj.type === "era")!.value;
  const year = parts.find((obj) => obj.type === "year")!.value;
  const month = parts.find((obj) => obj.type === "month")!.value;
  const day = parts.find((obj) => obj.type === "day")!.value;
  return `${era} ${year} 年 ${month} 月 ${day} 日`;
  */ 
 return "XXXX 年 X 月 X 日";
}

function dateString(date: Date): string {
  return date.toISOString();
}

function pastYearDays(targetDate: Date): string {
  const currentTime: Date = new Date();
  const interval: number = targetDate.getTime() - currentTime.getTime();
  return `${interval} msec?`;
}

type Row = {
  name: string;
  date: string;
  eraDate: string;
  age: string;
};

function CalcAge() {
  const date: Date = new Date();

  const dateTimeString = dateToJapaneseFormatString(date);
  const isoDateTimeString = dateString(date);

  const rows: Row[] = [
    {
      name: "現在時刻",
      date: isoDateTimeString,
      eraDate: dateTimeString,
      age: pastYearDays(date),
    },
  ];

  return (
    <React.Fragment>
      <h1>時間けいさん君</h1>
      作ってる途中
      <TableContainer component={Paper}>
        <Table sx={{ minWidth: 650 }}>
          <TableHead>
            <TableRow>
              <TableCell>名前</TableCell>
              <TableCell>西暦</TableCell>
              <TableCell>和暦</TableCell>
              <TableCell>経過年月</TableCell>
              <TableCell>操作</TableCell>
            </TableRow>
          </TableHead>
          <TableBody>
            {rows.map((row) => (
              <TableRow key={row.name}>
                <TableCell>{row.name}</TableCell>
                <TableCell>{row.date}</TableCell>
                <TableCell>{row.eraDate}</TableCell>
                <TableCell>{row.age}</TableCell>
                <TableCell>
                  <Button variant="contained" color="error">
                    削除
                  </Button>
                </TableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table>
      </TableContainer>
    </React.Fragment>
  );
}

export default CalcAge;
