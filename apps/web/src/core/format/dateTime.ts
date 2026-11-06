export function formatDateLocal(value: string | null | undefined): string {
  if (!value) {
    return "-";
  }

  const date = new Date(value);

  if (Number.isNaN(date.getTime())) {
    return "-";
  }

  const year = date.getFullYear();
  const month = pad2(date.getMonth() + 1);
  const day = pad2(date.getDate());

  return `${year}.${month}.${day}`;
}

export function formatYearMonthLocal(value: string | null | undefined): string {
  if (!value) {
    return "-";
  }

  const isoYearMonth = /^(\d{4})-(\d{2})/.exec(value);

  if (isoYearMonth) {
    return `${isoYearMonth[1]}-${isoYearMonth[2]}`;
  }

  const date = new Date(value);

  if (Number.isNaN(date.getTime())) {
    return "-";
  }

  const year = date.getFullYear();
  const month = pad2(date.getMonth() + 1);

  return `${year}-${month}`;
}

export function formatTimeLocal(value: string | null | undefined): string {
  if (!value) {
    return "-";
  }

  const date = new Date(value);

  if (Number.isNaN(date.getTime())) {
    return "-";
  }

  return `${pad2(date.getHours())}:${pad2(date.getMinutes())}`;
}

export function formatDateTimeLocal(value: string | null | undefined): string {
  if (!value) {
    return "-";
  }

  const date = new Date(value);

  if (Number.isNaN(date.getTime())) {
    return "-";
  }

  const year = date.getFullYear();
  const month = pad2(date.getMonth() + 1);
  const day = pad2(date.getDate());
  const hour = pad2(date.getHours());
  const minute = pad2(date.getMinutes());

  return `${year}.${month}.${day} ${hour}:${minute}`;
}

function pad2(value: number): string {
  return String(value).padStart(2, "0");
}
