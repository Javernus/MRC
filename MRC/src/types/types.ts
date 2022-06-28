export interface Group {
  id: number;
  name: string;
  password: string;
}

export interface Chat {
  groupId: number;
  time: number;
  name: string;
  message: string;
}
