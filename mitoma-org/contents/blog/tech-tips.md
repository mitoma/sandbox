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