import Database from "tauri-plugin-sql-api"
import type { Group, Chat } from "../types/types"

let db = null

const connect = async (): Promise<Database> => {
  if (db) return db

  try {
    return await Database.load('sqlite:mrc.db');
  } catch (e) {
    console.log(e);
  }
}

const getGroups = async (): Promise<Group[]> => {
  const db = await connect();
  return await db.select("SELECT * FROM groups")
}

const getChatById = async (id: number): Promise<Chat> => {
  const db = await connect();
  return await db.select("SELECT * FROM chats WHERE id = ?", [id])[0]
}

const getChats = async (): Promise<Chat[]> => {
  const db = await connect();
  return await db.select("SELECT * FROM chats")
}

const createGroup = async (group: Group): Promise<Group> => {
  const db = await connect();
  const qr = await db.execute("INSERT INTO groups (name, bio) VALUES ($1,$2)", [group.name, group.bio])

  return {
    id: qr.lastInsertId,
    name: group.name,
    bio: group.bio,
  }
}

const removeGroup = async (groupId: number): Promise<void> => {
  const db = await connect();
  await db.execute("DELETE FROM groups WHERE id = $1", [groupId])
}

const sendChat = async (chat: Chat): Promise<Chat> => {
  const db = await connect();
  const qr = await db.execute("INSERT INTO chats (groupId, name, message) VALUES ($1,$2,$3)", [chat.groupId, chat.name, chat.message])

  console.log("Time: ", (await getChatById(qr.lastInsertId)).time)

  return {
    id: qr.lastInsertId,
    groupId: chat.groupId,
    time: (await getChatById(qr.lastInsertId)).time,
    name: chat.name,
    message: chat.message,
  }
}

export default {
  connect,
  getGroups,
  getChats,
  createGroup,
  removeGroup,
  sendChat,
}
