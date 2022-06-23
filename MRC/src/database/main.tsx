import Database from "tauri-plugin-sql-api"
import type { Group, Chat } from "../types/types"

let db = null

const load = Database.load("sqlite:test.db").then((instance) => {
  db = instance
  console.log("database loaded", db)
  return db
})

const getGroups = async (): Promise<Group[]> => {
  await load
  return await db.select("SELECT * FROM groups")
}

const getChats = async (): Promise<Chat[]> => {
  await load
  return await db.select("SELECT * FROM chats")
}

const createGroup = async (group: Group): Promise<Group> => {
  await load
  console.log("Creating ", group)
  const { id: id } = await db.execute("INSERT INTO groups (name, bio) VALUES ($1, $2)", group.name, group.bio)

  return {
    id: id,
    name: group.name,
    bio: group.bio,
  }
}

const sendChat = async (chat: Chat): Promise<Chat> => {
  await load
  console.log("Sending ", chat)
  const { id: id, time: time } = await db.execute("INSERT INTO chats (group_id, name, message) VALUES ($1, $2, $3)", chat.groupId, chat.name, chat.message)

  return {
    id: id,
    groupId: chat.groupId,
    time: time,
    name: chat.name,
    message: chat.message,
  }
}

export {
  getGroups,
  getChats,
  createGroup,
  sendChat,
}
