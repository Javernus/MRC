import { Component, createSignal, createResource, For } from 'solid-js'
import './App.scss'
import './colours.scss'
import { Button, ChatItem, GroupItem, HamburgerX, Panel, InputField, Settings, Terminal } from './components'
import type { Group } from './types/types'
import DB from './database/main'
import cl from 'clsx'
import { listen } from '@tauri-apps/api/event'

let activeGroups = [1]

// Return the chat message with the latest timestamp.
const getLastChats = async (groups: Group[]) => {
  return await Promise.all(groups.map(async (group) => {
    const chat = await DB.getLastChat(group.id)
    if (chat["group_id"] === 0) return { groupId: group.id, message: "" }
    return {
      groupId: group.id,
      message: "<" + chat["name"] + "> " + chat["message"],
    }
  }))
}

const chatsFromGroup = async (group: Group) => {
  return await (await DB.getChats(group.id)).reverse()
}

const requestGroups = async (setGroups) => {
  const groups: Group[] = await DB.getGroups()
  setGroups(groups)
}

const App: Component = () => {
  let [plusMenu, setPlusMenu] = createSignal(false)
  let [groups, setGroups] = createSignal([])
  let [username, setUsername] = createSignal('')
  let [showChangeUsername, setShowChangeUsername] = createSignal(false)
  let [showJoinGroup, setShowJoinGroup] = createSignal(false)
  let [joinGroupError, setJoinGroupError] = createSignal(false)
  let [showCreateGroup, setShowCreateGroup] = createSignal(false)
  let [search, setSearch] = createSignal('')
  let [openGroup, setOpenGroup] = createSignal(null)
  let [showGroupInfo, setShowGroupInfo] = createSignal(false)
  let [chats, chatRefetch] = createResource(openGroup, chatsFromGroup)
  let [lastChats, lastChatsRefetch] = createResource(groups, getLastChats)
  let [groupName, setGroupName] = createSignal('')
  let [groupPassword, setGroupPassword] = createSignal('')

  const requestUsername = async () => {
    const uname = await DB.getUsername()
    setUsername(uname)
    if (uname === "") {
      setShowChangeUsername(true)
    }
  }


  const updateUsername = async () => {
    if (username() === '') {
      return
    }

    await DB.setUsername(username())
    setShowChangeUsername(false)
    setPlusMenu(false)
  }

  const searchGroups = (event) => {
    setSearch(event.target.value)
  }

  const createGroup = async () => {
    const group = await DB.createGroup(groupName(), groupPassword())
    setGroups([...groups(), group])

    setGroupName('')
    setGroupPassword('')

    setShowCreateGroup(false)
    setPlusMenu(false)
    setOpenGroup(group)
  }

  const joinGroup = async () => {
    if (groupName() === '') {
      setJoinGroupError(true)
      return
    }

    const group = await DB.joinGroup(groupName(), groupPassword())

    if (group.id === 0) {
      setJoinGroupError(true)
      return
    }

    setGroups([...groups(), group])

    setGroupName('')
    setGroupPassword('')

    setShowJoinGroup(false)
    setPlusMenu(false)
    setOpenGroup(group)
    setJoinGroupError(false)
  }

  const deleteGroup = async () => {
    setGroups(groups().filter(group => group.id !== openGroup().id))
    await DB.removeGroup(openGroup().id)
    setOpenGroup(null)
  }

  // Send the message using DB and add it to the chats array.
  const sendChat = async (message: string, groupId: number) => {
    await DB.sendChat(message, groupId)
    chatRefetch.refetch()
    lastChatsRefetch.refetch()
    scrollDown()
  }

  const getLastMessage = (groupId: number) => {
    let lastChat = lastChats().find(chat => chat.groupId === groupId)
    return lastChat ? lastChat.message : ''
  }

  requestGroups(setGroups)
  requestUsername()

  let chatElement

  const scrollDown = () => {
    setTimeout(() => chatElement.scrollTop = chatElement.scrollHeight, 25)
  }

  DB.receiver()

  const message = listen('refetch_chat', () => {
    chatRefetch.refetch()
    lastChatsRefetch.refetch()
  })

  return (
    <>
      <Panel visible>
        <div class='top-bar'>
          <HamburgerX
            size='2rem'
            isX={plusMenu()}
            onclick={() => {
                if (showCreateGroup() || showChangeUsername() || plusMenu() || showJoinGroup()) {
                  setPlusMenu(false)
                  setShowCreateGroup(false)
                  setShowChangeUsername(false)
                  setShowJoinGroup(false)
                  setJoinGroupError(false)
                  setGroupName('')
                  setGroupPassword('')
                  setJoinGroupError(false)
                } else {
                  setPlusMenu(true)
                }
              }
            }
          />
          <InputField placeholder="Search" oninput={searchGroups} />
        </div>
        <div class={cl('change-username', { 'change-username--visible': showChangeUsername() })}>
          <p class='change-username__header'>Set Username</p>
          <InputField placeholder="Username" value={username()} oninput={(event) => setUsername(event.target.value)} />
          <Button onclick={updateUsername} type="submit">Change</Button>
        </div>
        <div class={cl('join-group', { 'join-group--visible': showJoinGroup() })}>
          <p class='join-group__header'>Join Group</p>
          <InputField placeholder="Group Name" value={groupName()} oninput={(event) => { setGroupName(event.target.value); setJoinGroupError(false) }} error={joinGroupError()} />
          <InputField placeholder="Password" type="password" oninput={(event) => { setGroupPassword(event.target.value); setJoinGroupError(false) }} value={groupPassword()} error={joinGroupError()} />
          <Button onclick={joinGroup} type="submit">Join</Button>
        </div>
        <div class={cl('create-group', { 'create-group--visible': showCreateGroup() })}>
          <p class='create-group__header'>Create Group</p>
          <InputField placeholder="Group Name" oninput={(event) => setGroupName(event.target.value)} value={groupName()} />
          <InputField placeholder="Password" type="password" oninput={(event) => setGroupPassword(event.target.value)} value={groupPassword()} />
          <Button onclick={createGroup} type="submit">Create</Button>
        </div>
        <div class={cl('plus-menu', { 'plus-menu--visible': plusMenu() })}>
          <GroupItem
            name='Create Group'
            onclick={() => setShowCreateGroup(true)}
            button={true}
          >
            <svg width="1000rem" height="2.25rem" viewBox="0 0 100 100" style="transform: rotate(45deg)">
              <path fill="none" stroke="white" class="plus" d="M 20,29.000046 H 80.000231 C 80.000231,29.000046 94.498839,28.817352 94.532987,66.711331 94.543142,77.980673 90.966081,81.670246 85.259173,81.668997 79.552261,81.667751 75.000211,74.999942 75.000211,74.999942 L 25.000021,25.000058" stroke-dasharray="54 207" stroke-dashoffset="-144" stroke-width="5" />
              <path fill="none" stroke="white" class="plus" d="M 20,70.999954 H 80.000231 C 80.000231,70.999954 94.498839,71.182648 94.532987,33.288669 94.543142,22.019327 90.966081,18.329754 85.259173,18.331003 79.552261,18.332249 75.000211,25.000058 75.000211,25.000058 L 25.000021,74.999942" stroke-dasharray="54 207" stroke-dashoffset="-144" stroke-width="5" />
            </svg>
          </GroupItem>
          <GroupItem
            name='Join Group'
            onclick={() => setShowJoinGroup(true)}
            button={true}
          >
            <svg version="1.1" viewBox="-1 -1 54 54" width="1000rem" height="1.25rem">
            <path fill="white" stroke-width="2" stroke="white" d="M51.704,51.273L36.845,35.82c3.79-3.801,6.138-9.041,6.138-14.82c0-11.58-9.42-21-21-21s-21,9.42-21,21s9.42,21,21,21 c5.083,0,9.748-1.817,13.384-4.832l14.895,15.491c0.196,0.205,0.458,0.307,0.721,0.307c0.25,0,0.499-0.093,0.693-0.279 C52.074,52.304,52.086,51.671,51.704,51.273z M21.983,40c-10.477,0-19-8.523-19-19s8.523-19,19-19s19,8.523,19,19
              S32.459,40,21.983,40z"/>
            </svg>
          </GroupItem>
          <GroupItem
            name='Change Username'
            onclick={() => setShowChangeUsername(true)}
            button={true}
          >
            <svg width="1000rem" height="1.25rem" viewBox="0 0 456.645 456.645">
              <g>
                <g>
                  <path fill="white" d="M431.466,25.209c-33.61-33.61-88.01-33.615-121.625,0L32.192,302.859c-1.947,1.944-3.437,4.59-4.054,7.431L0.371,438.469
                    c-1.08,4.984,0.447,10.176,4.054,13.782c3.61,3.611,8.806,5.132,13.782,4.054l128.18-27.768c2.869-0.621,5.506-2.129,7.431-4.054
                    l277.649-277.649C464.998,113.302,464.998,58.742,431.466,25.209z M34.623,422.053l17.013-78.537l61.524,61.523L34.623,422.053z
                    M143.211,392.664l-79.199-79.199L307,70.477l79.199,79.2L143.211,392.664z M410.254,125.621l-2.842,2.842l-79.199-79.2
                    l2.842-2.842c21.864-21.864,57.31-21.887,79.199,0C432.088,68.257,432.088,103.786,410.254,125.621z"/>
                </g>
              </g>
            </svg>
          </GroupItem>
        </div>
        <div class='groups'>
          <For each={groups().filter(group => group.name.toLowerCase().includes(search().toLowerCase()))}>{(group: Group, index) =>
            <GroupItem
              name={group.name}
              colour={`hsl(${((120 - index()) % 360) * 5}, 40%, 70%)`}
              lastChat={getLastMessage(group.id)}
              status={activeGroups.includes(group.id) ? 'green' : 'yellow'}
              active={group === openGroup()}
              groupId={group.id}
              onclick={() => { setOpenGroup(group); scrollDown() }}
            />
          }</For>
        </div>
      </Panel>

      <Terminal
        ref={chatElement}
        disabled={!openGroup()}
        send={(message) => {sendChat(message, openGroup().id)}}
      >
        <For each={chats()}>{(chat) =>
          <ChatItem
            chat={chat}
          ></ChatItem>
        }</For>
      </Terminal>
      {
        !!openGroup() &&
        <Panel right fitContent visible={showGroupInfo()}>
          <Settings size='2rem' onclick={() => setShowGroupInfo(!showGroupInfo())} />
          <div class='top-bar'>
            <div class='group-icon'>
              <div class='group-icon__indicator group-icon__indicator--green' />
            </div>
            <div class='group-name'><p>{openGroup().name}</p></div>
          </div>
          <div class='delete-group' onclick={deleteGroup}>
            <p>Delete Group Data</p>
          </div>
        </Panel>
      }
    </>
  )
}

export default App
