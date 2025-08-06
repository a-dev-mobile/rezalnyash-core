package com.example.debug.engine.model;

/* loaded from: classes.dex */
public class ClientInfo {
    private String browser;
    private String countryIsoCode;
    private String device;
    private String deviceId;
    private String email;
    private String id;
    private String ip;
    private String language;
    private String location;
    private String os;
    private String parentId;
    private String plan;
    private Integer screenHeight;
    private Integer screenWidth;
    private String timeZone;
    private String version;

    public String getId() {
        return this.id;
    }

    public void setId(String str) {
        this.id = str;
    }

    public String getPlan() {
        return this.plan;
    }

    public void setPlan(String str) {
        this.plan = str;
    }

    public String getParentId() {
        return this.parentId;
    }

    public void setParentId(String str) {
        this.parentId = str;
    }

    public String getIp() {
        return this.ip;
    }

    public void setIp(String str) {
        this.ip = str;
    }

    public String getEmail() {
        return this.email;
    }

    public void setEmail(String str) {
        this.email = str;
    }

    public String getCountryIsoCode() {
        return this.countryIsoCode;
    }

    public void setCountryIsoCode(String str) {
        this.countryIsoCode = str;
    }

    public String getLocation() {
        return this.location;
    }

    public void setLocation(String str) {
        this.location = str;
    }

    public String getLanguage() {
        return this.language;
    }

    public void setLanguage(String str) {
        this.language = str;
    }

    public String getTimeZone() {
        return this.timeZone;
    }

    public void setTimeZone(String str) {
        this.timeZone = str;
    }

    public String getOs() {
        return this.os;
    }

    public void setOs(String str) {
        this.os = str;
    }

    public Integer getScreenWidth() {
        return this.screenWidth;
    }

    public void setScreenWidth(Integer num) {
        this.screenWidth = num;
    }

    public Integer getScreenHeight() {
        return this.screenHeight;
    }

    public void setScreenHeight(Integer num) {
        this.screenHeight = num;
    }

    public String getBrowser() {
        return this.browser;
    }

    public void setBrowser(String str) {
        this.browser = str;
    }

    public String getDevice() {
        return this.device;
    }

    public void setDevice(String str) {
        this.device = str;
    }

    public String getDeviceId() {
        return this.deviceId;
    }

    public void setDeviceId(String str) {
        this.deviceId = str;
    }

    public String getVersion() {
        return this.version;
    }

    public void setVersion(String str) {
        this.version = str;
    }
}
