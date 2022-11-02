package com.itheima.exception;

import sun.dc.pr.PRError;

/**
 * @version 1.0
 * @Author 李强
 * @Datetime 2022/8/23 8:48
 */
public class BusinessException extends RuntimeException{
    private Integer code;

    public Integer getCode() {
        return code;
    }

    public void setCode(Integer code) {
        this.code = code;
    }

    public BusinessException(Integer code, String message) {
        super(message);
        this.code = code;
    }

    public BusinessException(Integer code,String message, Throwable cause) {
        super(message, cause);
        this.code = code;
    }
}
