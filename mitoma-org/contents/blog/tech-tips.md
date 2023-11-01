# tips

## apt

### 指定したバージョンで固定する

最新版では都合が悪くて、バージョンの pinning を行いたい時の手順。(例はkubectl)

```sh
# apt upgrade で最新に上がらないようにする(hold 指定)
echo kubectl hold | sudo dpkg --set-selections

# apt upgrade で最新に上がるようにする(hold の解除)
echo kubectl install | sudo dpkg --set-selections

# 今の設定状況を確認する
dpkg --get-selections kubectl

# hold 状態で任意バージョンを指定してインストールする
# (hold 状態が解除されるので再度 hold が必要)
sudo apt install kubectl=1.22.11-00

# 利用可能バージョンを確認する
apt-cache showpkg kubectl | grep 1.22
```

## WSL2

### GUI アプリケーションを Windows のスタートメニューに出さない。

参考リンク: [https://learn.microsoft.com/ja-jp/windows/wsl/wsl-config](https://learn.microsoft.com/ja-jp/windows/wsl/wsl-config)

```
[wsl2]
guiApplications=false
```

### Docker を起動する時に cgroup 関連のマウントエラーが発生する

参考リンク

- [https://learn.microsoft.com/ja-jp/windows/wsl/wsl-config#systemd-support](https://learn.microsoft.com/ja-jp/windows/wsl/wsl-config#systemd-support)
- [WSLのIssue](https://github.com/microsoft/WSL/issues/9868)

WSL2 で docker を起動するため `.bashrc` には以下のように設定している。

```sh
# for docker-ce
# docker のインストールは docker 公式サイト通り
# visudo で以下の行を追加する
# mitoma  ALL=NOPASSWD:   /usr/sbin/service
docker_is_running=0
sudo service docker status 2>&1 > /dev/null || docker_is_running="$?"
if [ ! "$docker_is_running" = "0" ]; then
  sudo service docker start
fi
```

## GitHub

gh コマンドで Issue を取得する際に以下のように projectItems を指定すると権限が足りないと怒られることがある。

```sh
gh issue list --json projectItems
```

```
GraphQL: Your token has not been granted the required scopes to execute this query. The 'id' field requires one of the following scopes: ['read:project'], but your token has only been granted the: ['gist', 'read:org', 'repo', 'workflow'] scopes. Please modify your token's scopes at: https://github.com/settings/tokens., Your token has not been granted the required scopes to execute this query. The 'id' field requires one of the following scopes: ['read:project'], but your token has only been granted the: ['gist', 'read:org', 'repo', 'workflow'] scopes. Please modify your token's scopes at: https://github.com/settings/tokens., Your token has not been granted the required scopes to execute this query. The 'title' field requires one of the following scopes: ['read:project'], but your token has only been granted the: ['gist', 'read:org', 'repo', 'workflow'] scopes. Please modify your token's scopes at: https://github.com/settings/tokens.
```

[GitHubのDocument][] にあるように `gh auth login --scopes "project"` を実行してプロジェクトの権限もあるトークンを取得すればよい。

[GitHubのDocument]: https://docs.github.com/ja/issues/planning-and-tracking-with-projects/automating-your-project/using-the-api-to-manage-projects

## shell

### シンボリックリンクを実体に置き換えた状態でアーカイブする

**やりたい事**

Git のあるリビジョンをシンボリックリンクをすべて実体に置き換えた状態でアーカイブしたい。
前提としてシンボリックリンクはすべてリポジトリ内のファイルのみである。

**回答案**

```sh
mkdir archive-raw archive-deref
# まずはディレクトリに書き出して…
git archive --format=tar HEAD  | tar xpf - -C archive-raw/
# tar の dereference 使って別ディレクトリに書き出してやって…
tar chf - -C archive-raw . | tar xpf - -C archive-deref/
# archive-deref を tar に固めて完成！
tar chf archive.tar -C archive-raw .
```

## ffmpeg

### apng to gif

それなりに画質を維持した状態で apng から gif に変換する。

```sh
ffmpeg -i from.png -filter_complex "[0:v] split [a][b];[a] palettegen [p];[b][p] paletteuse" to.gif
```
