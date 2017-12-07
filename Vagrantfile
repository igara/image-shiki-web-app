# -*- mode: ruby -*-
# vi: set ft=ruby :
def install_plugin(plugin)
    system "vagrant plugin install #{plugin}" unless Vagrant.has_plugin? plugin
  end
  install_plugin('vagrant-vbguest')
  
  Vagrant.configure("2") do |config|
    config.vm.box = "ubuntu/xenial64"
    config.vbguest.auto_update = true
  
    config.vm.provider "virtualbox" do |vb|
      vb.memory = 1024
    end

    config.vm.network :private_network, ip: "192.168.33.50"
    config.vm.network :forwarded_port, guest: 1234, host: 1234
    config.ssh.forward_agent = true
    # sync project files
    config.vm.synced_folder "./", "/vagrant", create: true, owner: "ubuntu", group: "ubuntu"
    config.vm.synced_folder "./", "/var/www", create: true, owner: "ubuntu", group: "ubuntu"
    config.vm.provision :shell, :path => "./vagrant/provision.sh", :privileged => false
    # 起動二回目以降は上記をコメント化し下記を適応してください
    # config.vm.provision :shell, :path => "./vagrant/start.sh", :privileged => false
  end
  