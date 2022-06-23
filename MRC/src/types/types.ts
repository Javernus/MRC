export interface Group {
  id: number;
  name: string;
  bio: string;
}

export interface Chat {
  id: number;
  groupId: number;
  time: number;
  name: string;
  message: string;
}
